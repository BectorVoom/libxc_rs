//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1199/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1199<F: Float>(t15123: F, t15125: F, t15128: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18977: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F, t19019: F) -> F {
    let t19021 = -F::cast_from(0.33547222222222222222e0_f64) * t18906 + F::new(0.12077e1) * t18911 - F::cast_from(0.40256666666666666666e0_f64) * t18915 + F::new(0.16504875e0) * t18951 - F::cast_from(0.18396666666666666667e0_f64) * t15123 - F::cast_from(0.40256666666666666668e0_f64) * t15125 + t15128 - F::new(0.181155e1) * t18928 + F::new(0.12077e1) * t18932 - F::cast_from(0.20128333333333333333e0_f64) * t18939 + t18977 + F::new(0.19419375e1) * t18980 - F::new(0.258925e1) * t18982 - F::new(0.1294625e1) * t18985 - F::cast_from(0.412621875e-1_f64) * t18988 + F::new(0.16504875e0) * t18990 + F::new(0.82524375e-1) * t18993 + F::new(0.258925e1) * t18995 + F::cast_from(0.67094444444444444443e-1_f64) * t18919 - F::cast_from(0.20128333333333333333e0_f64) * t18924 + F::cast_from(0.10064166666666666667e0_f64) * t18934 + t19019;
    t19021
}
