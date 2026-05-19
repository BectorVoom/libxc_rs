//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1083/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1083<F: Float>(t30137: F, t7585: F, t8525: F, t2030: F, t301: F, t4262: F, t8484: F, t2060: F, t372: F, t8927: F, t34837: F, t34840: F, t34841: F, t34843: F, t34844: F, t34846: F, t34847: F, t34848: F, t34849: F, t34851: F, t34853: F, t34856: F, t34857: F, t34859: F, t34862: F) -> F {
    let t34865 = t7585 * t30137 * t8525;
    let t34866 = F::cast_from(0.14291339372689912324e-3_f64) * t34865;
    let t34869 = t2030 * t4262 * t8484 * t301;
    let t34873 = t2060 * t8927 * t8484 * t372;
    let t34875 = -t34837 + t34840 - F::cast_from(0.10289764348336736873e-1_f64) * t34841 + t34843 + F::cast_from(0.17149607247227894789e-2_f64) * t34844 + t34846 - t34847 + t34848 - F::cast_from(0.56606566121287473722e-2_f64) * t34849 + F::cast_from(0.80031500487063509015e-2_f64) * t34851 - F::cast_from(0.80031500487063509015e-2_f64) * t34853 + t34856 + F::cast_from(0.17149607247227894789e-2_f64) * t34857 + F::cast_from(0.85748036236139473944e-3_f64) * t34859 - F::cast_from(0.21437009059034868486e-3_f64) * t34862 - t34866 - F::new(0.4584375e-1) * t34869 - F::new(0.4584375e-1) * t34873;
    t34875
}
