//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 380/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk380<F: Float>(t1796: F, t1779: F, t367: F, t78: F, t1071: F, t46: F, t1072: F, t374: F, t1176: F, t1182: F, t1185: F, t1126: F, t1127: F, t1131: F, t1138: F, t1153: F, t1161: F, t1772: F, t1778: F, t1780: F, t1790: F, t1795: F, t242: F, t4: F, t55: F, t624: F, t631: F, t637: F, t638: F, t98: F) -> F {
    let t1797 = F::new(1.0) / t1796;
    let t1798 = t1779 * t1797;
    let t1804 = t78 * t367;
    let t1808 = t46 * t1071;
    let t1809 = t1072 * t374;
    let t1812 = t1176 * t374;
    let t1815 = t46 * t1182;
    let t1816 = t1072 * t1185;
    let t1819 = -F::new(0.70981924444444444442e-3) * t4 * t98 * t242 - F::new(0.34246666666666666666e-1) * t1127 * t1772 * t631 - F::new(2.0) * t1778 * t1780 + F::new(1.0) * t624 * t1790 + F::new(0.32164683177870697974e2) * t1795 * t1798 + t1126 + t1131 + t1138 - t1153 - t1161 - F::new(0.24415406715670879921e-3) * t4 * t98 * t55 - F::new(0.10843580882781524214e-1) * t1127 * t1804 * t638 - F::new(0.11696446794910408142e1) * t1808 * t1809 + F::new(0.58482233974552040708e0) * t637 * t1812 + F::new(0.17315755899375863299e2) * t1815 * t1816;
    t1819
}
