//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1256;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta426<F: Float>(t1036: F, t21483: F, t1041: F, t13969: F, t21511: F, t10413: F, t10422: F, t21531: F, t21486: F, t3130: F, t21565: F, t3070: F, t21126: F, t2970: F, t973: F, t21569: F, t42488: F, t10231: F, t21122: F, t21689: F, t225: F, t21669: F, t21684: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t70766, t70792, t70800, t70805, t70846) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1256::<F>(t1036, t21483, t1041, t13969, t21511, t10413, t10422, t21531, t21486, t3130, t21565, t3070);
        let (t70867, t70912, t70929, t70978, t70980, t70987) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1257::<F>(t21126, t2970, t973, t21569, t3070, t42488, t10231, t21122, t21689, t225, t21669, t21684);
    (t70766, t70792, t70800, t70805, t70846, t70867, t70912, t70929, t70978, t70980, t70987)
}
