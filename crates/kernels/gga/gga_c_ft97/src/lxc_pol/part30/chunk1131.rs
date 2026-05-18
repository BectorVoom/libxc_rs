//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1131/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1131<F: Float>(t150436: F, t28603: F, t153304: F, t28652: F, t153060: F, t28660: F, t127649: F, t35462: F, t31535: F, t127560: F, t127614: F, t142725: F, t14729: F, t150902: F, t153091: F, t153169: F, t153241: F, t153256: F, t153272: F, t153280: F, t153285: F, t19039: F, t19107: F, t19135: F, t28615: F, t28676: F, t291: F, t31462: F, t33436: F, t33948: F, t35924: F, t35941: F, t35961: F, t4061: F, t683: F, t7003: F, t812: F, t821: F) -> F {
    let t153338 = t28603 * t150436;
    let t153342 = t28652 * t153304;
    let t153350 = t28660 * t153060;
    let t153362 = t127649 * t35462;
    let t153365 = t31535 * t35462;
    let t153368 = F::new(0.12081826776807659559e1) * t31462 * t153272 - F::new(0.27369475924647479992e1) * t19039 * t35924 * t812 - F::new(0.45306850413028723348e0) * t4061 * t291 * t35961 - F::new(0.54738951849294959985e1) * t19107 * t153256 - F::new(0.45306850413028723348e0) * t14729 * t153241 - F::new(0.10069900737806194568e-1) * t153338 - F::new(0.18125821328051150223e0) * t28660 * t153169 + F::new(0.6041940442683716741e-1) * t153342 - F::new(0.24008161480400406449e0) * t142725 * t33436 * t683 * t28615 - F::new(0.53351469956445347664e-1) * t33948 * t150902 - F::new(0.6041940442683716741e-1) * t153350 - F::new(0.20527106943485609994e0) * t19135 * t153091 - F::new(0.82108427773942439976e0) * t127614 * t35941 + F::new(0.41054213886971219988e0) * t127560 * t35941 - F::new(0.91656519086197144464e-1) * t28676 * t153280 + F::new(0.45828259543098572232e-1) * t7003 * t153285 + F::new(0.45828259543098572232e-1) * t153362 * t812 - F::new(0.22914129771549286116e-1) * t153365 * t821;
    t153368
}
