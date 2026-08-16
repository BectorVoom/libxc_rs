//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1299/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1299(t5664: f64, t67159: f64, t58021: f64, t46278: f64, t67177: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t39483: f64, t40741: f64, t40743: f64, t40748: f64, t40760: f64, t40764: f64, t40766: f64, t40772: f64, t4314: f64, t67154: f64, t67235: f64) -> (f64, f64, f64, f64, f64) {
    let t75879 = t5664 * t5664;
    let t75884 = 4.0_f64 * t67159;
    let t75885 = 0.35089341735807877242e1_f64 * t58021;
    let t75886 = 0.65061487801810439052e-1_f64 * t46278;
    let t75887 = 48.0_f64 * t67177;
    let t75891 = -6.0_f64 * t193 * t202 * t40772 * t75879 + 24.0_f64 * t1484 * t4314 * t67235 - 4.0_f64 * t1530 * t1877 * t67154 + t39483 - t40741 - t40743 + t40748 + t40760 + t40764 + t40766 + t75884 - t75885 + t75886 + t75887;
    (t75884, t75885, t75886, t75887, t75891)
}
