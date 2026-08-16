//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2393/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2393(t10704: f64, t4395: f64, t10702: f64, t2793: f64, t10524: f64, t10603: f64, t10717: f64, t10724: f64, t10734: f64, t10756: f64, t10765: f64, t14271: f64, t14276: f64, t14337: f64, t14369: f64, t14459: f64, t14466: f64, t1580: f64, t2906: f64, t2924: f64, t2930: f64, t41826: f64, t41981: f64, t42111: f64, t42113: f64, t42123: f64, t4416: f64, t4438: f64, t4475: f64, t48883: f64, t48890: f64, t49068: f64, t49071: f64, t950: f64) -> (f64, f64) {
    let t49072 = t4395 * t10704;
    let t49075 = 0.1551780387578202009e4_f64 * t10702 * t49072 * t2793;
    let t49076 = -6.0_f64 * t14276 * t10734 + 0.96491876992155210402e2_f64 * t14271 * t10717 - 6.0_f64 * t41981 * t4416 + 0.96491876992155210402e2_f64 * t42123 * t4438 + 0.17315859105681463759e2_f64 * t2930 * t4475 * t10603 + 0.91082604192152556044e5_f64 * t42111 * t1580 * t42113 * t10524 + 0.51947577317044391277e2_f64 * t14337 * t10724 - 0.12304822629859687989e5_f64 * t41826 * t14369 * t10524 + 0.51947577317044391277e2_f64 * t2930 * t48883 * t950 + 0.51947577317044391277e2_f64 * t2930 * t14459 * t2924 + 0.30762056574649219973e4_f64 * t10756 * t48890 * t2906 + 18.0_f64 * t10765 * t14466 - t49068 - t49071 - t49075;
    (t49075, t49076)
}
