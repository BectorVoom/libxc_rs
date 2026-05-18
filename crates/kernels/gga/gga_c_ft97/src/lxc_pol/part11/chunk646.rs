//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 646/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk646<F: Float>(t139: F, t8997: F, t527: F, t2031: F, t3347: F, t1992: F, t555: F, t133: F, t140: F, t1993: F, t1995: F, t2001: F, t2002: F, t2059: F, t2071: F, t3392: F, t399: F, t543: F, t550: F, t5818: F, t7926: F, t7936: F, t8866: F, t8869: F, t8874: F, t8877: F, t8883: F, t8885: F, t8895: F, t8908: F, t8909: F, t8932: F, t8935: F, t8937: F) -> (F, F, F, F, F) {
    let t8998 = t139 * t8997;
    let t8999 = t527 * t8998;
    let t9001 = t3347 * t2031;
    let t9003 = t1992 * t555;
    let t9005 = -F::new(6.0) * t2001 * t8866 + F::new(12.0) * t2001 * t8869 * t2059 + F::new(0.10862994854660402308e0) * t3392 * t8874 + F::new(6.0) * t3392 * t8877 * t2071 - F::new(0.32588984563981206924e0) * t5818 * t8874 + F::new(0.72490960660845957359e1) * t1995 * t8883 * t8885 - F::new(0.72490960660845957359e1) * t527 * t8883 * t8885 - F::new(6.0) * t2001 * t2002 * t2071 - F::new(0.3624548033042297868e1) * t8895 * t399 + F::new(0.3624548033042297868e1) * t1993 * t399 - F::new(0.27734402270309446394e2) * t140 * t7926 - F::new(0.28056686626142231644e2) * t543 * t7936 + F::new(0.55468804540618892788e2) * t543 * t7926 - F::new(6.0) * t133 * t8908 * t8909 - t133 * t550 * t8932 + F::new(0.16294492281990603462e0) * t8935 * t8937 + F::new(2.0) * t8999 + F::new(6.0) * t9001 - F::new(6.0) * t9003;
    (t8998, t8999, t9001, t9003, t9005)
}
