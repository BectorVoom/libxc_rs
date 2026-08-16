//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 646/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk646(t139: f64, t8997: f64, t527: f64, t2031: f64, t3347: f64, t1992: f64, t555: f64, t133: f64, t140: f64, t1993: f64, t1995: f64, t2001: f64, t2002: f64, t2059: f64, t2071: f64, t3392: f64, t399: f64, t543: f64, t550: f64, t5818: f64, t7926: f64, t7936: f64, t8866: f64, t8869: f64, t8874: f64, t8877: f64, t8883: f64, t8885: f64, t8895: f64, t8908: f64, t8909: f64, t8932: f64, t8935: f64, t8937: f64) -> (f64, f64, f64, f64, f64) {
    let t8998 = t139 * t8997;
    let t8999 = t527 * t8998;
    let t9001 = t3347 * t2031;
    let t9003 = t1992 * t555;
    let t9005 = -6.0_f64 * t2001 * t8866 + 12.0_f64 * t2001 * t8869 * t2059 + 0.10862994854660402308e0_f64 * t3392 * t8874 + 6.0_f64 * t3392 * t8877 * t2071 - 0.32588984563981206924e0_f64 * t5818 * t8874 + 0.72490960660845957359e1_f64 * t1995 * t8883 * t8885 - 0.72490960660845957359e1_f64 * t527 * t8883 * t8885 - 6.0_f64 * t2001 * t2002 * t2071 - 0.3624548033042297868e1_f64 * t8895 * t399 + 0.3624548033042297868e1_f64 * t1993 * t399 - 0.27734402270309446394e2_f64 * t140 * t7926 - 0.28056686626142231644e2_f64 * t543 * t7936 + 0.55468804540618892788e2_f64 * t543 * t7926 - 6.0_f64 * t133 * t8908 * t8909 - t133 * t550 * t8932 + 0.16294492281990603462e0_f64 * t8935 * t8937 + 2.0_f64 * t8999 + 6.0_f64 * t9001 - 6.0_f64 * t9003;
    (t8998, t8999, t9001, t9003, t9005)
}
