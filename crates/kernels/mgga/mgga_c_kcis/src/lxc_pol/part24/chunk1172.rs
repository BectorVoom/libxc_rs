//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1172/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1172<F: Float>(t3668: F, t7807: F, t10497: F, t2183: F, t11061: F, t7788: F, t7790: F, t1071: F, t3622: F, t26954: F, t27076: F, t26996: F, t993: F) -> (F, F, F, F, F, F) {
    let t92576 = t7807 * t3668;
    let t92581 = t2183 * t10497;
    let t92600 = t7788 * t11061 * t7790;
    let t92651 = t3622 * t1071;
    let t92657 = t27076 * t26954;
    let t92693 = t993 * t26996;
    (t92576, t92581, t92600, t92651, t92657, t92693)
}
