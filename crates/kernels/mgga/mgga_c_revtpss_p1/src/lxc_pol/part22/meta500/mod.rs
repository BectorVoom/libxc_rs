//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2233;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta500<F: Float>(t3075: F, t5004: F, t359: F, t4930: F, t999: F, t1043: F, t1089: F, t4757: F, t3291: F, t4772: F, t1678: F, t3133: F, t15957: F, t4976: F, t1024: F, t1087: F, t11782: F, t11788: F, t12122: F, t12127: F, t12149: F, t16427: F, t16433: F, t16436: F, t16440: F, t16443: F, t1685: F, t1692: F, t3043: F, t3223: F, t3278: F, t3287: F, t3299: F, t3313: F, t4954: F, t4961: F, t4981: F, t4988: F, t5005: F) -> (F, F, F, F, F, F, F, F) {
        let (t16446, t16449, t16450, t16458, t16461, t16465) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2233::<F>(t3075, t5004, t359, t4930, t999, t1043, t1089, t4757, t3291, t4772, t1678, t3133);
        let (t16468, t16475) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2234::<F>(t15957, t4976, t1024, t1087, t11782, t11788, t12122, t12127, t12149, t16427, t16433, t16436, t16440, t16443, t16446, t16450, t16458, t16461, t16465, t1685, t1692, t3043, t3223, t3278, t3287, t3299, t3313, t4954, t4961, t4981, t4988, t5005);
    (t16446, t16449, t16450, t16458, t16461, t16465, t16468, t16475)
}
