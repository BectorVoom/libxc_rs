//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1083/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1083<F: Float>(t27557: F, t6034: F, t697: F, t108445: F, t108447: F, t108448: F, t108454: F, t108456: F, t108460: F, t108464: F, t108468: F, t108472: F, t108476: F, t108479: F, t108487: F, t108494: F, t108495: F, t1120: F, t13522: F, t232: F, t24265: F, t24361: F, t24372: F, t27522: F, t27547: F, t3699: F, t684: F, t96465: F, t96614: F, t98545: F) -> (F,) {
    let t108501 = 0.29693535778629056444e-4 * t6034 * t697 * t27557;
    let t108502 = -t108445 + 0.60548059007656442388e-3 * t108447 * t108448 * t27522 * t684 - 0.98910212891072794759e-5 * t108454 - 0.44540303667943584666e-4 * t6034 * t232 * t108456 + 0.89019191601965515283e-5 * t24372 * t232 * t108460 - 0.14836531933660919214e-4 * t24372 * t232 * t108464 + 0.24710505058474293383e-6 * t96465 * t232 * t108468 + 0.53448364401532301599e-4 * t6034 * t232 * t108472 - 0.17263005832038132093e-5 * t96614 * t108476 - 0.51074886703703703704e-1 * t24361 * t98545 * t3699 * t108479 + 0.23754828622903245156e-2 * t24265 * t1120 * t13522 + 0.51074886703703703704e-1 * t108487 * t98545 * t27547 * t684 + t108494 - 0.44540303667943584666e-3 * t24265 * t232 * t108495 - t108501;
    (t108502,)
}
