//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 251/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk251<F: Float>(t169: F, t274: F, t301: F, t784: F, t125: F, t143: F, t145: F, t153: F, t156: F, t163: F, t164: F, t168: F, t171: F, t242: F, t245: F, t279: F, t281: F, t296: F, t299: F, t475: F, t481: F, t523: F, t526: F, t530: F, t536: F, t549: F, t555: F, t559: F, t684: F, t692: F, t702: F, t706: F, t738: F, t744: F, t745: F, t753: F, t755: F, t761: F, t765: F, t769: F, t770: F, t776: F, t778: F) -> (F, F) {
    let t788 = F::cast_from(0.54045904796391420712e-1_f64) * t169 * t784 * t274 * t301;
    let t793 = F::cast_from(3.0_f64) * t475 * t143 * t481 + t523 * t526 + (t530 - F::cast_from(0.31505407223141117834e-1_f64) * t536 * t164 - t549 - t555 + t559 - F::cast_from(0.53884053046145740922e-2_f64) * t169 * t171 * t684 * t163) * t125 + (t692 - F::cast_from(0.83762820535504401876e-1_f64) * t536 * t242 - t702 + t706 - F::cast_from(0.11938374665504764976e-1_f64) * t168 * t245 * t738 - t744 + F::cast_from(0.42708890021612718669e0_f64) * t153 * t156 * t745) * t279 + t753 - F::cast_from(0.11974234010254609094e-1_f64) * t281 * t755 - t761 - t765 + (t769 - F::cast_from(0.31835665774679373271e-1_f64) * t169 * t770 * t242 - t776 - t778 + F::cast_from(0.533250677421793803e-1_f64) * t145 * t745) * t296 - t788 + F::cast_from(0.20267214298646782767e-1_f64) * t169 * t299 * t745 * t301;
    (t788, t793)
}
