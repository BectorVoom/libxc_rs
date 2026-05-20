//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1992/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1992<F: Float>(t102567: F, t108615: F, t108617: F, t108619: F, t108623: F, t108625: F, t108627: F, t108629: F, t94554: F, t96358: F, t96359: F, t98283: F, t98285: F) -> F {
    let t109839 = -F::cast_from(0.30488190661738479625e-3_f64) * t94554 + t108615 / F::new(8.0) - t108617 / F::new(2.0) + t108619 / F::new(4.0) + t102567 - t98283 - t96358 - t96359 - F::cast_from(0.14457274399185490173e-3_f64) * t98285 + F::cast_from(0.28582678745379824648e-4_f64) * t108623 + F::cast_from(0.10164000561857065645e-2_f64) * t108625 - F::cast_from(0.80031500487063509015e-1_f64) * t108627 + F::cast_from(0.16006300097412701803e-1_f64) * t108629;
    t109839
}
