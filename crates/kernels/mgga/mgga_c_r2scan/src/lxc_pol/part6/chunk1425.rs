//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1425/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1425<F: Float>(t18908: F, t18916: F, t18920: F, t18922: F, t18930: F, t22603: F, t22606: F, t22609: F, t22612: F, t23711: F, t23715: F, t23718: F, t2060: F, t2062: F, t7006: F, t5998: F, t7872: F) -> (F, F, F) {
    let t26895 = t23711 - t18908 + t23715 - t18916 - 0.2025780996e0 * t22603 - t18920 + t18922 + t18930 + t23718 + 0.4051561992e0 * t22606 + 0.8103123984e0 * t22609 - 0.675260332e-1 * t22612;
    let t26899 = t2060 * t7006 * t2062;
    let t26901 = t7872 * t5998;
    (t26895, t26899, t26901)
}
