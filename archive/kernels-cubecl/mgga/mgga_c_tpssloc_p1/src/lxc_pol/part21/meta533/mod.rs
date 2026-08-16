//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2192;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta533<F: Float>(t1049: F, t5866: F, t1060: F, t1022: F, t11066: F, t5928: F, t3201: F, t4649: F, t1629: F, t11060: F, t4684: F, t5936: F, t4673: F, t1058: F, t1061: F, t11034: F, t11037: F, t11046: F, t11059: F, t11065: F, t14618: F, t14651: F, t1630: F, t18081: F, t18083: F, t18086: F, t18089: F, t18094: F, t3180: F, t3186: F, t3200: F, t4674: F, t5929: F, t5937: F, t5939: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t18099, t18100, t18103, t18104, t18107, t18108, t18111, t18112, t18117) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2192::<F>(t1049, t5866, t1060, t1022, t11066, t5928, t3201, t4649, t1629, t11060, t4684, t5936);
        let (t18121, t18124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2193::<F>(t4673, t5936, t1058, t1061, t11034, t11037, t11046, t11059, t11065, t14618, t14651, t1630, t18081, t18083, t18086, t18089, t18094, t18100, t18104, t18108, t18112, t18117, t3180, t3186, t3200, t4674, t5929, t5937, t5939);
    (t18099, t18100, t18103, t18104, t18107, t18108, t18111, t18112, t18117, t18121, t18124)
}
