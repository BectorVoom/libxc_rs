//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 347/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk347(t159: f64, t285: f64, t991: f64, t171: f64, t1052: f64, t1072: f64, t1076: f64, t125: f64, t145: f64, t153: f64, t156: f64, t163: f64, t164: f64, t168: f64, t169: f64, t242: f64, t245: f64, t279: f64, t281: f64, t296: f64, t299: f64, t301: f64, t475: f64, t526: f64, t530: f64, t549: f64, t555: f64, t559: f64, t692: f64, t702: f64, t706: f64, t744: f64, t753: f64, t761: f64, t765: f64, t769: f64, t776: f64, t778: f64, t788: f64, t968: f64, t988: f64, t992: f64) -> (f64, f64, f64) {
    let t1083 = t991 * t159 * t285;
    let t1086 = t171 * t991;
    let t1098 = 3.0_f64 * t475 * t968 + t988 * t526 + (t530 - 0.31505407223141117834e-1_f64 * t992 * t164 - t549 - t555 + t559 - 0.53884053046145740922e-2_f64 * t169 * t171 * t1052 * t163) * t125 + (t692 - 0.83762820535504401876e-1_f64 * t992 * t242 - t702 + t706 - 0.11938374665504764976e-1_f64 * t168 * t245 * t1072 - t744 + 0.42708890021612718669e0_f64 * t153 * t156 * t1076) * t279 + t753 - 0.11974234010254609094e-1_f64 * t281 * t1083 - t761 - t765 + (t769 - 0.31835665774679373271e-1_f64 * t169 * t1086 * t242 - t776 - t778 + 0.533250677421793803e-1_f64 * t145 * t1076) * t296 - t788 + 0.20267214298646782767e-1_f64 * t169 * t299 * t1076 * t301;
    (t1083, t1086, t1098)
}
