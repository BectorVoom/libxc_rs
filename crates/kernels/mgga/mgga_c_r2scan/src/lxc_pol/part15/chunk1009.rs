//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1009/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1009<F: Float>(t3308: F, t8066: F, t574: F, t2651: F, t3309: F, t10810: F, t2608: F, t10698: F, t3588: F, t10870: F, t10886: F, t10892: F, t10895: F, t10897: F, t10902: F, t10905: F, t11835: F) -> (F, F, F) {
    let t11837 = t3308 * t8066;
    let t11838 = t574 * t11837;
    let t11840 = t2651 * t3309;
    let t11842 = t10810 * t2608;
    let t11843 = t574 * t11842;
    let t11845 = t10698 * t3588;
    let t11852 = -F::new(0.23287303101564395623e-1) * t10870 - F::new(0.43341108700271342816e-1) * t11835 - F::new(0.43341108700271342816e-1) * t11838 - F::new(0.43341108700271342816e-1) * t11840 + F::new(0.11557628986739024751e0) * t11843 + F::new(0.64025200389650807209e-1) * t11845 + F::new(0.11557628986739024751e0) * t10886 + F::new(0.34672886960217074253e0) * t10892 + F::new(0.27439371595564631661e-2) * t10895 - F::new(0.97574405393827830186e-2) * t10897 - t10902 + F::new(0.23287303101564395623e-1) * t10905;
    (t11837, t11842, t11852)
}
