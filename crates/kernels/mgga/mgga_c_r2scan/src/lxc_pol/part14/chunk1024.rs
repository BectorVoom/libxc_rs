//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1024/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1024<F: Float>(t3602: F, t37755: F, t6064: F, t3606: F, t6087: F, t2195: F, t37754: F, t37769: F, t7606: F, t11842: F, t1584: F, t3597: F, t6182: F, t11711: F, t6425: F, t1055: F, t7918: F) -> (F, F, F, F, F, F, F, F) {
    let t39927 = t37755 * t3602 * t6064;
    let t39930 = t37755 * t3606 * t6087;
    let t39935 = t2195 * t37754;
    let t39937 = t39935 * t3606 * t6064;
    let t39939 = t37769 * t7606;
    let t39941 = t1584 * t11842;
    let t39945 = t6182 * t3597;
    let t39947 = t6425 * t11711;
    let t39951 = t7918 * t1055;
    (t39927, t39930, t39937, t39939, t39941, t39945, t39947, t39951)
}
