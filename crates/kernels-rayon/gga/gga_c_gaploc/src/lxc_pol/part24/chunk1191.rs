//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1191/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1191(t31889: f64, t2268: f64, t6320: f64, t6509: f64, t8097: f64, t20117: f64, t2854: f64, t10246: f64, t6313: f64, t10124: f64, t10153: f64, t1064: f64, t31863: f64, t31865: f64, t31869: f64, t31870: f64, t31879: f64, t31881: f64, t31883: f64, t31886: f64, t3818: f64, t3822: f64) -> f64 {
    let t31890 = 0.11856252764865062333e-2_f64 * t31889;
    let t31894 = 0.34146007962811379518e0_f64 * t2268 * t6320 * t8097 * t6509;
    let t31898 = 0.34146007962811379518e0_f64 * t2268 * t6320 * t2854 * t20117;
    let t31900 = 0.53116012386595479252e0_f64 * t6313 * t10246;
    let t31901 = t31863 + t31865 + t31869 - 0.56910013271352299198e-1_f64 * t3822 * t1064 * t31870 + 0.7588001769513639893e-1_f64 * t3818 * t10124 + 0.15176003539027279786e0_f64 * t6313 * t10153 - t31879 + t31881 - t31883 + t31886 - t31890 - t31894 - t31898 - t31900;
    t31901
}
