//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1083/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1083(t30137: f64, t7585: f64, t8525: f64, t2030: f64, t301: f64, t4262: f64, t8484: f64, t2060: f64, t372: f64, t8927: f64, t34837: f64, t34840: f64, t34841: f64, t34843: f64, t34844: f64, t34846: f64, t34847: f64, t34848: f64, t34849: f64, t34851: f64, t34853: f64, t34856: f64, t34857: f64, t34859: f64, t34862: f64) -> f64 {
    let t34865 = t7585 * t30137 * t8525;
    let t34866 = 0.14291339372689912324e-3_f64 * t34865;
    let t34869 = t2030 * t4262 * t8484 * t301;
    let t34873 = t2060 * t8927 * t8484 * t372;
    let t34875 = -t34837 + t34840 - 0.10289764348336736873e-1_f64 * t34841 + t34843 + 0.17149607247227894789e-2_f64 * t34844 + t34846 - t34847 + t34848 - 0.56606566121287473722e-2_f64 * t34849 + 0.80031500487063509015e-2_f64 * t34851 - 0.80031500487063509015e-2_f64 * t34853 + t34856 + 0.17149607247227894789e-2_f64 * t34857 + 0.85748036236139473944e-3_f64 * t34859 - 0.21437009059034868486e-3_f64 * t34862 - t34866 - 0.4584375e-1_f64 * t34869 - 0.4584375e-1_f64 * t34873;
    t34875
}
