//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1018/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1018(t141160: f64, t150351: f64, t150355: f64, t150358: f64, t150359: f64, t150364: f64, t150367: f64, t150372: f64, t150378: f64, t218: f64, t27512: f64, t27558: f64, t27562: f64, t27646: f64, t27658: f64, t33380: f64, t33424: f64, t33426: f64, t33427: f64, t33428: f64, t3751: f64, t41: f64, t55109: f64, t6057: f64, t684: f64, t7205: f64, t79528: f64, t96694: f64) -> f64 {
    let t150393 = 0.36398255417420433543e-3_f64 * t33424 * t150351 * t33428 - 0.25537443351851851852e-1_f64 * t150355 * t6057 + 0.7825932155388508152e-2_f64 * t150358 * t33426 * t150359 * t684 - 0.85124811172839506173e-2_f64 * t150364 - 0.20676097475611486194e-4_f64 * t150367 * t27558 - 0.68872808489893002037e-5_f64 * t150367 * t27562 - 0.11738898233082762228e-1_f64 * t141160 * t33426 * t150372 * t27646 - 0.22705522127871165896e-3_f64 * t27658 * t150378 - 0.68246728907663312894e-4_f64 * t33424 * t33426 * t33427 * t3751 + 0.3967677301665257484e-6_f64 * t79528 * t96694 * t41 * t218 * t7205 * t55109 + 0.51074886703703703704e-1_f64 * t33380 * t27512;
    t150393
}
