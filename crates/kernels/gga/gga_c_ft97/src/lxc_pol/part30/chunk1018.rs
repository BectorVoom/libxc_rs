//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1018/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1018<F: Float>(t141160: F, t150351: F, t150355: F, t150358: F, t150359: F, t150364: F, t150367: F, t150372: F, t150378: F, t218: F, t27512: F, t27558: F, t27562: F, t27646: F, t27658: F, t33380: F, t33424: F, t33426: F, t33427: F, t33428: F, t3751: F, t41: F, t55109: F, t6057: F, t684: F, t7205: F, t79528: F, t96694: F) -> F {
    let t150393 = F::cast_from(0.36398255417420433543e-3_f64) * t33424 * t150351 * t33428 - F::cast_from(0.25537443351851851852e-1_f64) * t150355 * t6057 + F::cast_from(0.7825932155388508152e-2_f64) * t150358 * t33426 * t150359 * t684 - F::cast_from(0.85124811172839506173e-2_f64) * t150364 - F::cast_from(0.20676097475611486194e-4_f64) * t150367 * t27558 - F::cast_from(0.68872808489893002037e-5_f64) * t150367 * t27562 - F::cast_from(0.11738898233082762228e-1_f64) * t141160 * t33426 * t150372 * t27646 - F::cast_from(0.22705522127871165896e-3_f64) * t27658 * t150378 - F::cast_from(0.68246728907663312894e-4_f64) * t33424 * t33426 * t33427 * t3751 + F::cast_from(0.3967677301665257484e-6_f64) * t79528 * t96694 * t41 * t218 * t7205 * t55109 + F::cast_from(0.51074886703703703704e-1_f64) * t33380 * t27512;
    t150393
}
