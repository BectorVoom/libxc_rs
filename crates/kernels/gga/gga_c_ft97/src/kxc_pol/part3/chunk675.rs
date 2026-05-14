//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 675/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk675<F: Float>(t15846: F, t419: F, t11273: F, t15746: F, t1725: F, t4484: F, t173: F, t4483: F, t1527: F, t15752: F, t11280: F, t15756: F, t4488: F, t4487: F, t15763: F, t3088: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15847 = t419 * t15846;
    let t15849 = t11273 * t15746;
    let t15850 = t419 * t15849;
    let t15852 = t1725 * t4484;
    let t15854 = t173 * t4483;
    let t15855 = t419 * t15854;
    let t15857 = t1527 * t15752;
    let t15858 = t419 * t15857;
    let t15860 = t11280 * t15756;
    let t15861 = t419 * t15860;
    let t15863 = t1725 * t4488;
    let t15865 = t173 * t4487;
    let t15866 = t419 * t15865;
    let t15868 = t3088 * t15763;
    (t15847, t15850, t15852, t15855, t15858, t15861, t15863, t15866, t15868)
}
