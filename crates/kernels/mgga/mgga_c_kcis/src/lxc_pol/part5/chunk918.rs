//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 918/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk918<F: Float>(t2675: F, t2683: F, t2366: F, t2375: F, t678: F, t859: F, t47: F, t8680: F, t8656: F, t680: F, t8698: F, t194: F, t2679: F) -> (F, F, F, F, F, F, F) {
    let t8798 = t2675 * t2683;
    let t8808 = t2366 * t2375;
    let t8809 = t8808 * t678;
    let t8812 = t859 * t2366;
    let t8815 = t47 * t8680;
    let t8816 = t8656 * t2375;
    let t8819 = t8698 * t680;
    let t8823 = F::new(1.0) / t2679 / t194;
    (t8798, t8809, t8812, t8815, t8816, t8819, t8823)
}
