//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1022/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1022<F: Float>(t12766: F, t12782: F, t12798: F, t12809: F, t797: F, t1048: F, t499: F, t12033: F, t3579: F, t1044: F, t3781: F, t11206: F, t11215: F, t11866: F, t11876: F, t11886: F, t12587: F, t12589: F, t12591: F, t12593: F, t12596: F, t12599: F) -> (F, F, F, F, F, F) {
    let t12811 = t12766 + t12782 + t12798 + t12809;
    let t12812 = t12811 * t797;
    let t12814 = t1048 * t499 * t12812;
    let t12815 = t12814 / F::new(4.0);
    let t12816 = t3579 * t12033;
    let t12817 = t12816 / F::new(2.0);
    let t12818 = t3781 * t1044;
    let t12819 = F::new(2.0) * t12818;
    let t12828 = -t11206 - F::new(4.0) / F::new(3.0) * t11866 - t12587 / F::new(2.0) + t12589 / F::new(4.0) - t12591 / F::new(4.0) + t12593 + F::new(4.0) / F::new(3.0) * t11876 - F::new(3.0) / F::new(2.0) * t12596 - F::new(8.0) / F::new(3.0) * t11886 + t12599 / F::new(2.0) - t11215;
    (t12811, t12812, t12815, t12817, t12819, t12828)
}
