//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1439/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1439<F: Float>(t122940: F, t9740: F, t118391: F, t118393: F, t118405: F, t121831: F, t121834: F, t121838: F, t121862: F, t121882: F, t121885: F, t121889: F, t121893: F, t121901: F, t121903: F, t20: F, t2003: F, t2801: F, t2807: F, t8831: F) -> (F,) {
    let t123156 = t9740 * t122940;
    let t123158 = 0.51588271604938271605e-2 * t121831 - 0.77382407407407407408e-2 * t121834 + 0.77382407407407407407e-3 * t121838 - 0.17411041666666666666e-2 * t121862 + t118391 + t118393 + t118405 - 0.50925925925925925926e-1 * t2801 * t2003 * t8831 * t20 * t2807 - 0.23214722222222222222e-2 * t121882 - 0.23214722222222222222e-2 * t121885 + 0.51588271604938271604e-3 * t121889 + 0.38691203703703703703e-3 * t121893 + 0.15476481481481481481e-2 * t121901 + 0.11349419753086419753e-1 * t121903 - 0.34722222222222222223e-2 * t123156;
    (t123158,)
}
