//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1193/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1193(t1246: f64, t14985: f64, t1235: f64, t5011: f64, t5072: f64, t5079: f64, t5068: f64, t5075: f64, t11883: f64, t3507: f64, t1755: f64, t11871: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14986 = t14985 * t1246;
    let t14988 = t1235 * t5011;
    let t14989 = t14988 * t1246;
    let t14992 = t5072 * t5079;
    let t14997 = t5075 * t5068;
    let t15000 = t11883 * t3507;
    let t15001 = t1755 * t15000;
    let t15004 = t5072 * t5068;
    let t15009 = t1755 * t11871;
    (t14986, t14989, t14992, t14997, t15001, t15004, t15009)
}
