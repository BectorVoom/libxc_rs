//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1106/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1106<F: Float>(t26639: F, t32233: F, t1009: F, t1014: F, t104722: F, t104860: F, t138746: F, t138773: F, t138794: F, t138799: F, t138854: F, t138857: F, t138879: F, t145171: F, t145516: F, t147248: F, t147258: F, t147274: F, t147279: F, t147291: F, t147298: F, t147308: F, t16762: F, t23839: F, t26638: F, t26705: F, t32140: F, t32241: F, t32764: F, t34873: F, t378: F, t40087: F, t94401: F, t94524: F) -> (F, F) {
    let t147310 = t32233 * t26639;
    let t147319 = -F::new(0.14500657062440920178e1) * t23839 * t147274 + F::new(0.6041940442683716741e-1) * t147279 + F::new(0.53351469956445347664e-1) * t32764 * t145516 - F::new(0.24008161480400406448e0) * t138773 * t32140 * t378 * t26638 + F::new(0.48016322960800812896e0) * t138879 * t32140 * t378 * t16762 + F::new(0.54377463984153450669e0) * t104722 * t147291 + F::new(0.18125821328051150223e0) * t104860 * t34873 + F::new(0.18125821328051150223e0) * t23839 * t147298 + F::new(0.82108427773942439976e0) * t40087 * t147258 + F::new(0.45828259543098572232e-1) * t138854 * t1009 - F::new(0.22914129771549286116e-1) * t138857 * t1014 - t138794 + F::new(0.53351469956445347664e-1) * t138799 - F::new(0.10069900737806194568e-1) * t147308 - F::new(0.36251642656102300446e0) * t94401 * t147310 - F::new(0.80027204934668021496e-1) * t138746 * t32241 * t145171 * t26705 - F::new(0.6041940442683716741e-1) * t94524 * t147248;
    (t147310, t147319)
}
