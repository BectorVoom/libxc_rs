//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta501<F: Float>(t21643: F, t3188: F, t11060: F, t21637: F, t11066: F, t3201: F, t5866: F, t1629: F, t1058: F, t11046: F, t11059: F, t11065: F, t14608: F, t14618: F, t1610: F, t1630: F, t1632: F, t18086: F, t21481: F, t21615: F, t21618: F, t21623: F, t21627: F, t21635: F, t21638: F, t3186: F, t3200: F, t353: F, t384: F, t4669: F, t5903: F, t5929: F, t5933: F, t5937: F, t5939: F, t5941: F) -> (F, F, F, F, F, F, F) {
        let (t21644, t21647, t21650, t21653, t21656, t21657, t21662) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1936::<F>(t21643, t3188, t11060, t21637, t11066, t3201, t5866, t1629, t1058, t11046, t11059, t11065, t14608, t14618, t1610, t1630, t1632, t18086, t21481, t21615, t21618, t21623, t21627, t21635, t21638, t3186, t3200, t353, t384, t4669, t5903, t5929, t5933, t5937, t5939, t5941);
    (t21644, t21647, t21650, t21653, t21656, t21657, t21662)
}
