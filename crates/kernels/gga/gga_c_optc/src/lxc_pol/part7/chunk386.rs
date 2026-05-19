//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 386/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk386<F: Float>(t1849: F, t1867: F, t1761: F, t1772: F, t1787: F, t1795: F, t1803: F, t1809: F, t1811: F, t1821: F, t1826: F, t1829: F, t1834: F, t1838: F, t1842: F, t1848: F, t1850: F, t1860: F, t1865: F, t209: F, t4: F, t566: F, t573: F, t581: F, t588: F, t71: F, t84: F) -> (F, F) {
    let t1868 = t1849 * t1867;
    let t1871 = -F::cast_from(0.70981924444444444442e-3_f64) * t4 * t1772 * t71 - F::cast_from(0.34246666666666666666e-1_f64) * t209 * t1803 * t573 - F::new(2.0) * t1809 * t1811 + F::new(1.0) * t566 * t1821 + F::cast_from(0.32164683177870697974e2_f64) * t1826 * t1829 + t1834 + t1838 + t1761 - t1787 - t1795 - F::cast_from(0.24415406715670879921e-3_f64) * t4 * t1772 * t84 - F::cast_from(0.10843580882781524214e-1_f64) * t209 * t1842 * t588 - F::cast_from(0.11696446794910408142e1_f64) * t1848 * t1850 + F::cast_from(0.58482233974552040708e0_f64) * t581 * t1860 + F::cast_from(0.17315755899375863299e2_f64) * t1865 * t1868;
    (t1868, t1871)
}
