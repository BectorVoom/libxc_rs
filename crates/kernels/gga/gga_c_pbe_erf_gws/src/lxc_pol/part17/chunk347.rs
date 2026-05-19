//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 347/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk347<F: Float>(t159: F, t285: F, t991: F, t171: F, t1052: F, t1072: F, t1076: F, t125: F, t145: F, t153: F, t156: F, t163: F, t164: F, t168: F, t169: F, t242: F, t245: F, t279: F, t281: F, t296: F, t299: F, t301: F, t475: F, t526: F, t530: F, t549: F, t555: F, t559: F, t692: F, t702: F, t706: F, t744: F, t753: F, t761: F, t765: F, t769: F, t776: F, t778: F, t788: F, t968: F, t988: F, t992: F) -> (F, F, F) {
    let t1083 = t991 * t159 * t285;
    let t1086 = t171 * t991;
    let t1098 = F::new(3.0) * t475 * t968 + t988 * t526 + (t530 - F::cast_from(0.31505407223141117834e-1_f64) * t992 * t164 - t549 - t555 + t559 - F::cast_from(0.53884053046145740922e-2_f64) * t169 * t171 * t1052 * t163) * t125 + (t692 - F::cast_from(0.83762820535504401876e-1_f64) * t992 * t242 - t702 + t706 - F::cast_from(0.11938374665504764976e-1_f64) * t168 * t245 * t1072 - t744 + F::cast_from(0.42708890021612718669e0_f64) * t153 * t156 * t1076) * t279 + t753 - F::cast_from(0.11974234010254609094e-1_f64) * t281 * t1083 - t761 - t765 + (t769 - F::cast_from(0.31835665774679373271e-1_f64) * t169 * t1086 * t242 - t776 - t778 + F::cast_from(0.533250677421793803e-1_f64) * t145 * t1076) * t296 - t788 + F::cast_from(0.20267214298646782767e-1_f64) * t169 * t299 * t1076 * t301;
    (t1083, t1086, t1098)
}
