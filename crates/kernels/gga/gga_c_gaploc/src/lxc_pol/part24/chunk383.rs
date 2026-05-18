//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 383/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk383<F: Float>(t1819: F, t231: F, t1163: F, t643: F, t4: F, t1167: F, t1074: F, t646: F, t1178: F, t1186: F, t1126: F, t1131: F, t1138: F, t1153: F, t1161: F) -> (F, F, F, F, F, F, F) {
    let t1820 = t231 * t1819;
    let t1822 = F::new(0.24415406715670879921e-3) * t643 * t1163;
    let t1823 = t231 * t4;
    let t1825 = F::new(0.10843580882781524214e-1) * t1823 * t1167;
    let t1827 = F::new(0.11696446794910408142e1) * t646 * t1074;
    let t1829 = F::new(0.58482233974552040708e0) * t646 * t1178;
    let t1831 = F::new(0.17315755899375863299e2) * t646 * t1186;
    let t1832 = -t1126 - t1131 - t1138 + t1153 + t1161 + t1820 + t1822 + t1825 + t1827 - t1829 - t1831;
    (t1820, t1822, t1825, t1827, t1829, t1831, t1832)
}
