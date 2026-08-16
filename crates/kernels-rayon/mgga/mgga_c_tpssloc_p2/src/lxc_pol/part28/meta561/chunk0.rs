//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1833/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1833(t1408: f64, t2553: f64, t10143: f64, t606: f64, t25374: f64, t25365: f64, t868: f64, t25373: f64, t58009: f64, t4255: f64, t22960: f64, t59580: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t86764 = t1408 * t2553;
    let t86770 = t10143 * t606;
    let t86771 = t86770 * t25374;
    let t86781 = t25365 * t868;
    let t86782 = t25373 * t86781;
    let t86794 = t25373 * t58009;
    let t86797 = t4255 * t868;
    let t86798 = t22960 * t86797;
    let t86803 = t22960 * t59580;
    (t86764, t86771, t86781, t86782, t86794, t86797, t86798, t86803)
}
