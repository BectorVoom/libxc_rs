//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 909/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk909(t30889: f64, t1988: f64, t7792: f64, t7796: f64, t7799: f64, t1078: f64, t1980: f64, t1982: f64, t1983: f64, t1997: f64, t3036: f64, t3213: f64) -> (f64, f64, f64, f64, f64) {
    let t30890 = 0.12862205435420921092e-2_f64 * t30889;
    let t30891 = t1988 * t7792;
    let t30893 = t7799 * t7796;
    let t30901 = t1980 * t1982 * t1078 * t1983;
    let t30904 = t3036 * t1997 * t3213;
    (t30890, t30891, t30893, t30901, t30904)
}
