//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1136/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1136<F: Float>(t3965: F, t9299: F, t1105: F, t2051: F, t15097: F, t945: F, t1172: F, t1211: F, t319: F, t4233: F, t6854: F, t321: F, t13756: F, t14161: F, t14852: F, t2182: F, t2494: F, t3189: F, t3946: F, t4062: F, t4120: F, t4194: F, t52079: F, t52105: F, t52775: F, t52782: F, t52837: F, t52841: F, t52870: F, t810: F) -> (F, F, F) {
    let t54734 = t3965 * t9299;
    let t54753 = t1105 * t2051;
    let t54766 = t15097 * t945;
    let t54778 = t1172 * t319 * t1211;
    let t54792 = t4233 * t6854;
    let t54797 = 2.0 * t321 * t54766;
    let t54798 = 12.0 * t13756 * t14161 * t3189 + 6.0 * t13756 * t14852 * t2182 + 6.0 * t14161 * t2494 * t3946 + 2.0 * t2051 * t4062 * t54792 - 6.0 * t3946 * t4120 * t52782 - 3.0 * t3946 * t4120 * t52837 + 6.0 * t3946 * t54766 * t810 - 6.0 * t4062 * t52105 * t52870 + 6.0 * t4194 * t52841 + 12.0 * t52775 * t54778 + 6.0 * t52079 + t54797;
    (t54734, t54753, t54798)
}
