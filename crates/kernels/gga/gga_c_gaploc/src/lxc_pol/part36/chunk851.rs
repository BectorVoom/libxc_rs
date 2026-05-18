//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 851/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk851<F: Float>(t34777: F, t901: F, t35106: F, t40219: F, t40225: F, t41915: F, t41919: F, t41922: F, t41927: F, t41930: F, t41933: F, t41935: F, t41938: F, t41941: F, t41942: F, t41945: F, t41948: F, t41950: F, t41952: F, t41954: F, t41958: F) -> F {
    let t41960 = t34777 * t901;
    let t41962 = t35106 * t901;
    let t41964 = t41915 + t41919 + t41922 + F::new(0.76685851907841499352e0) * t40219 - t41927 - t41930 + t41933 + F::new(0.30674340763136599741e2) * t41935 + F::new(0.95334639871601137787e0) * t41938 + t41941 + F::new(0.71500979903700853338e0) * t41942 + t41945 - F::new(0.1533717038156829987e1) * t40225 - t41948 - t41950 - t41952 + F::new(0.51123901271894332901e0) * t41954 - F::new(0.89376224879626066674e-1) * t41958 + F::new(0.29792074959875355558e-1) * t41960 + F::new(0.29792074959875355558e-1) * t41962;
    t41964
}
