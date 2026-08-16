//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1280;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta381<F: Float>(t21040: F, t6638: F, t3626: F, t471: F, t5351: F, t6429: F, t6425: F, t6421: F, t12787: F, t23842: F, t5268: F, t1042: F, t1261: F, t17448: F, t17605: F, t17792: F, t1782: F, t21213: F, t21283: F, t21285: F, t21287: F, t3625: F, t5373: F, t6640: F, t6659: F, t6663: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24786, t24787, t24792) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1280::<F>(t21040, t6638, t3626, t471, t5351);
        let (t24793, t24794, t24797, t24798, t24803, t24804, t24807, t24808, t24815) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1281::<F>(t24792, t6429, t3626, t6425, t6421, t12787, t23842, t5268, t1042, t1261, t17448, t17605, t17792, t1782, t21213, t21283, t21285, t21287, t24787, t3625, t5373, t6640, t6659, t6663);
    (t24786, t24787, t24792, t24793, t24794, t24797, t24798, t24803, t24804, t24807, t24808, t24815)
}
