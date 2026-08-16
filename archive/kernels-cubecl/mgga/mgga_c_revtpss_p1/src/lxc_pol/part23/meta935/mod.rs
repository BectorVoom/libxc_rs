//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta935 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3074;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta935<F: Float>(t422: F, t81286: F, t81304: F, t20473: F, t5192: F, t24407: F, t3520: F, t1196: F, t5206: F, t20391: F, t20394: F, t81254: F, t81257: F, t81259: F, t81261: F, t81264: F, t81266: F, t5184: F, t68680: F, t1187: F, t6534: F, t1757: F, t58708: F, t20400: F, t5198: F, t20887: F, t58665: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t81307, t81309, t81313, t81315, t81317, t81318) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3074::<F>(t422, t81286, t81304, t20473, t5192, t24407, t3520, t1196, t5206, t20391, t20394, t81254, t81257, t81259, t81261, t81264, t81266);
        let (t81322, t81326, t81328, t81330, t81333) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3075::<F>(t1196, t5184, t68680, t1187, t6534, t1757, t58708, t20400, t5198, t20887, t5192, t58665);
    (t81307, t81309, t81313, t81315, t81317, t81318, t81322, t81326, t81328, t81330, t81333)
}
