//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1158/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1158<F: Float>(t31992: F, t31993: F, t4816: F, t32000: F, t4820: F, t1065: F, t1646: F, t1695: F, t1042: F, t120181: F, t120238: F, t120244: F, t120259: F, t120263: F, t120288: F, t120305: F, t120307: F, t120321: F, t120495: F, t1671: F, t3092: F, t32014: F, t33754: F, t33768: F, t33827: F, t33832: F, t4781: F, t4825: F, t906: F, t93437: F) -> F {
    let t126483 = t31992 * t31993 * t4816;
    let t126487 = t32000 * t4820;
    let t126494 = t1065 * t1646;
    let t126501 = t1065 * t1695;
    let t126508 = -F::cast_from(0.18822977838986977999e-3_f64) * t32014 * t3092 * t33768 * t906 - F::cast_from(0.37645955677973955999e-3_f64) * t120321 * t3092 * t33754 * t906 + F::cast_from(0.66110807482757352571e-3_f64) * t120288 * t33832 - F::cast_from(0.82638509353446690713e-4_f64) * t126483 - F::cast_from(0.19833242244827205771e-2_f64) * t120181 * t33827 - F::cast_from(0.24791552806034007213e-3_f64) * t126487 - F::cast_from(0.24791552806034007214e-3_f64) * t120244 + F::cast_from(0.56468933516960933998e-3_f64) * t120305 * t120307 * t4781 * t93437 - F::cast_from(0.3718732920905101082e-3_f64) * t120238 * t1042 * t126494 * t906 + F::cast_from(0.3718732920905101082e-3_f64) * t120259 * t4825 + F::cast_from(0.24791552806034007213e-3_f64) * t120263 * t1042 * t126501 * t906 - F::cast_from(0.3718732920905101082e-3_f64) * t120495 * t1671;
    t126508
}
