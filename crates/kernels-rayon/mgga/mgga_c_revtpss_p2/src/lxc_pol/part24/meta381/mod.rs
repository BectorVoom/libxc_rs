//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1280;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta381(t21040: f64, t6638: f64, t3626: f64, t471: f64, t5351: f64, t6429: f64, t6425: f64, t6421: f64, t12787: f64, t23842: f64, t5268: f64, t1042: f64, t1261: f64, t17448: f64, t17605: f64, t17792: f64, t1782: f64, t21213: f64, t21283: f64, t21285: f64, t21287: f64, t3625: f64, t5373: f64, t6640: f64, t6659: f64, t6663: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24786, t24787, t24792) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1280(t21040, t6638, t3626, t471, t5351);
        let (t24793, t24794, t24797, t24798, t24803, t24804, t24807, t24808, t24815) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1281(t24792, t6429, t3626, t6425, t6421, t12787, t23842, t5268, t1042, t1261, t17448, t17605, t17792, t1782, t21213, t21283, t21285, t21287, t24787, t3625, t5373, t6640, t6659, t6663);
    (t24786, t24787, t24792, t24793, t24794, t24797, t24798, t24803, t24804, t24807, t24808, t24815)
}
