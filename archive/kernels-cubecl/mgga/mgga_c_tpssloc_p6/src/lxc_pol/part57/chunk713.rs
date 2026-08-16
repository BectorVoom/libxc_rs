//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 713/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk713<F: Float>(t225: F, t23592: F, t11094: F, t1958: F, t2752: F, t28: F, t111: F, t2022: F, t22468: F, t2094: F, t531: F, t7025: F, t9239: F) -> (F, F, F, F, F, F, F) {
    let t23696 = t23592 * t225;
    let t23742 = t1958 * t11094;
    let t23788 = t2752 * t28;
    let t23880 = t2022 * t111;
    let t23912 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t22468;
    let t23957 = t531 * t2094;
    let t23963 = t9239 * t7025;
    (t23696, t23742, t23788, t23880, t23912, t23957, t23963)
}
