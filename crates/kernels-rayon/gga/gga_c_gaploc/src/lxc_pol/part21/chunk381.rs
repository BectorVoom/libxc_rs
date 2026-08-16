//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 381/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk381(t1796: f64, t1779: f64, t367: f64, t78: f64, t1071: f64, t46: f64, t1072: f64, t374: f64, t1176: f64, t1182: f64, t1185: f64, t1126: f64, t1127: f64, t1131: f64, t1138: f64, t1153: f64, t1161: f64, t1772: f64, t1778: f64, t1780: f64, t1790: f64, t1795: f64, t242: f64, t4: f64, t55: f64, t624: f64, t631: f64, t637: f64, t638: f64, t98: f64) -> f64 {
    let t1797 = 1.0_f64 / t1796;
    let t1798 = t1779 * t1797;
    let t1804 = t78 * t367;
    let t1808 = t46 * t1071;
    let t1809 = t1072 * t374;
    let t1812 = t1176 * t374;
    let t1815 = t46 * t1182;
    let t1816 = t1072 * t1185;
    let t1819 = -0.70981924444444444442e-3_f64 * t4 * t98 * t242 - 0.34246666666666666666e-1_f64 * t1127 * t1772 * t631 - 2.0_f64 * t1778 * t1780 + 1.0_f64 * t624 * t1790 + 0.32164683177870697974e2_f64 * t1795 * t1798 + t1126 + t1131 + t1138 - t1153 - t1161 - 0.24415406715670879921e-3_f64 * t4 * t98 * t55 - 0.10843580882781524214e-1_f64 * t1127 * t1804 * t638 - 0.11696446794910408142e1_f64 * t1808 * t1809 + 0.58482233974552040708e0_f64 * t637 * t1812 + 0.17315755899375863299e2_f64 * t1815 * t1816;
    t1819
}
