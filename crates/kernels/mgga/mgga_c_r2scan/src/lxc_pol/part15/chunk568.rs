//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 568/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk568<F: Float>(t5: F, t898: F, t736: F, t725: F, t41: F, t585: F, t955: F, t159: F, t617: F, t1856: F, t1863: F, t1866: F, t1874: F, t1875: F, t1885: F, t1888: F, t1897: F, t1901: F, t1904: F, t1910: F, t1913: F, t1916: F, t2037: F, t216: F, t2483: F) -> (F, F, F, F, F) {
    let t2788 = t898 * t5;
    let t2789 = t2788 * t736;
    let t2794 = t898 * t725;
    let t2795 = t41 * t2794;
    let t2798 = t955 * t585;
    let t2799 = t159 * t2798;
    let t2800 = t2799 * t617;
    let t2802 = -t1856 - F::new(0.54217906501508699211e-2) * t2789 - F::new(0.21973736767207854065e-2) * t2483 * t216 + F::new(0.1350520664e0) * t1863 - t2795 - F::new(0.571528e-1) * t1866 - t1874 + F::new(4.0) * t1875 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916 + F::new(0.84681398666666666666e-3) * t2800 - t2037;
    (t2788, t2794, t2798, t2799, t2802)
}
