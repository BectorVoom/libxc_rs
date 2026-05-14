//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 893/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk893<F: Float>(t27733: F, t33365: F, t150327: F, t3766: F, t1113: F, t2344: F, t27569: F, t33380: F, t17836: F, t24389: F, t52: F, t668: F, t2247: F, t27511: F, t33403: F, t141160: F, t150351: F, t218: F, t27512: F, t27558: F, t27562: F, t27646: F, t27658: F, t33424: F, t33426: F, t33427: F, t33428: F, t3751: F, t41: F, t55109: F, t6057: F, t684: F, t7205: F, t79528: F, t96694: F) -> (F, F, F) {
    let t150355 = t27733 * t33365;
    let t150358 = t3766 * t150327;
    let t150359 = t2344 * t1113;
    let t150364 = t33380 * t27569;
    let t150367 = t17836 * t24389 * t52;
    let t150372 = t2344 * t668;
    let t150378 = t33403 * t2247 * t27511;
    let t150393 = 0.36398255417420433543e-3 * t33424 * t150351 * t33428 - 0.25537443351851851852e-1 * t150355 * t6057 + 0.7825932155388508152e-2 * t150358 * t33426 * t150359 * t684 - 0.85124811172839506173e-2 * t150364 - 0.20676097475611486194e-4 * t150367 * t27558 - 0.68872808489893002037e-5 * t150367 * t27562 - 0.11738898233082762228e-1 * t141160 * t33426 * t150372 * t27646 - 0.22705522127871165896e-3 * t27658 * t150378 - 0.68246728907663312894e-4 * t33424 * t33426 * t33427 * t3751 + 0.3967677301665257484e-6 * t79528 * t96694 * t41 * t218 * t7205 * t55109 + 0.51074886703703703704e-1 * t33380 * t27512;
    (t150372, t150378, t150393)
}
