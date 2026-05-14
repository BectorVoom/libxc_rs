//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 883/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk883<F: Float>(t193: F, t27942: F, t1425: F, t4003: F, t27882: F, t6009: F, t1131: F, t771: F, t6008: F, t1403: F, t24213: F, t24221: F, t24224: F, t27925: F, t27927: F, t27930: F, t27934: F, t27936: F, t27939: F, t5996: F, t6068: F, t6745: F, t6844: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27943 = t193 * t27942;
    let t27946 = t1425 * t4003;
    let t27947 = t193 * t27946;
    let t27952 = t27882 * t6009;
    let t27953 = t193 * t27952;
    let t27956 = t771 * t1131;
    let t27957 = t6008 * t27956;
    let t27958 = t193 * t27957;
    let t27961 = t5996 * t6844 / 6.0 + 4.0 * t27925 - t27927 / 18.0 - t27930 / 18.0 + t24213 - t24221 / 18.0 - t24224 / 18.0 - 2.0 * t27934 + t27936 / 54.0 + t1403 * t27939 / 6.0 + t1403 * t27943 / 6.0 + t1403 * t27947 / 6.0 + t6745 * t6068 / 6.0 - t1403 * t27953 / 3.0 - t1403 * t27958 / 3.0;
    (t27943, t27946, t27947, t27952, t27953, t27956, t27957, t27958, t27961)
}
