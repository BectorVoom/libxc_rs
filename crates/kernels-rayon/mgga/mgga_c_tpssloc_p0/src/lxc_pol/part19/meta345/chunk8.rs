//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1242/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1242(t225: f64, t9520: f64, t10049: f64, t10054: f64, t10055: f64, t10076: f64, t10084: f64, t10097: f64, t10101: f64, t10103: f64, t10104: f64, t10112: f64, t10116: f64, t218: f64, t22997: f64, t23175: f64, t252: f64, t259: f64, t2597: f64, t2617: f64, t2633: f64, t2679: f64, t2684: f64, t2718: f64, t2720: f64, t2729: f64, t2733: f64, t2736: f64, t2738: f64, t2743: f64, t40890: f64, t40891: f64, t40895: f64, t40904: f64, t40909: f64, t40917: f64, t41230: f64, t41388: f64, t41490: f64, t41495: f64, t41520: f64, t41549: f64, t4182: f64, t4281: f64, t4291: f64, t812: f64, t829: f64, t852: f64, t855: f64, t858: f64, t860: f64, t861: f64, t865: f64, t866: f64, t9584: f64, t9590: f64, t9612: f64, t9632: f64, t9976: f64, t9981: f64) -> f64 {
    let t41554 = t9520 * t225;
    let t41580 = 24.0_f64 * t855 * t40890 * t40891 - t855 * t858 * (24.0_f64 * t812 * t10054 * t9981 - 6.0_f64 * t812 * t10076 * t2684 + 12.0_f64 * t812 * t40895 * t2633 + 8.0_f64 * t4281 * t40909 * t4182 - 24.0_f64 * t812 * t40917 * t9976 + 24.0_f64 * t2617 * t10055 + 24.0_f64 * t2617 * t10084 + 12.0_f64 * t9612 * t2729 - 6.0_f64 * t9612 * t2736 - 4.0_f64 * t40904 * t861 + t41495 - 6.0_f64 * t812 * t10076 * t2679 - 6.0_f64 * t4291 * t10097 * t2684 + 36.0_f64 * t4281 * t22997 * t9632 + 24.0_f64 * t4281 * t23175 * t9632 - 4.0_f64 * t4291 * t40909 * t829 - t812 * t860 * t41388 - 4.0_f64 * t812 * t41520 * t829 - 4.0_f64 * t2617 * t10101 - 12.0_f64 * t9612 * t2733 - 6.0_f64 * t9612 * t2738 + t41549) - 12.0_f64 * t41554 * t866 - 24.0_f64 * t2597 * t10112 + 24.0_f64 * t2597 * t10116 + 8.0_f64 * t855 * t2718 * t10103 * t865 - 6.0_f64 * t9590 * t2743 - 4.0_f64 * t2597 * t10104 - 6.0_f64 * t10049 * t2743 + t41230 * t252 * t259 + 12.0_f64 * t9590 * t2720 + 4.0_f64 * t9584 * t852 * t259 + t218 * t41490 * t259;
    t41580
}
