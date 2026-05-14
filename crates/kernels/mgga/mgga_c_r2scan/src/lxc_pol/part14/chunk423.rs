//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 423/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk423<F: Float>(t1835: F, t224: F, t1691: F, t720: F, t234: F, t712: F, t717: F, t749: F, t732: F, t741: F, t750: F, t625: F, t626: F, t645: F, t1794: F, t1796: F, t1803: F, t1806: F, t1808: F, t1812: F, t1816: F, t1825: F, t1829: F, t1833: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1836 = t1835 * t224;
    let t1837 = t720 * t1691;
    let t1838 = t1836 * t1837;
    let t1840 = 0.10389515463408878255e3 * t234 * t1838;
    let t1841 = t717 * t712;
    let t1842 = t1841 * t749;
    let t1844 = 0.34631718211362927518e2 * t234 * t1842;
    let t1845 = t732 * t741;
    let t1847 = t732 * t750;
    let t1851 = 0.35616666666666666666e-1 * t625 * t626 * t645;
    let t1852 = -2.0 * t1794 + 8.0 * t1796 - t1803 - t1806 - t1808 + t1812 + t1816 + t1825 - t1829 - t1833 - t1840 + t1844 - 0.23392894490538584828e1 * t1845 + 0.34631718211362927518e2 * t1847 + t1851;
    (t1836, t1837, t1838, t1840, t1841, t1842, t1844, t1845, t1847, t1851, t1852)
}
