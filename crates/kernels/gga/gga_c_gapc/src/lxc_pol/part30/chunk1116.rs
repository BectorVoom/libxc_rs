//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1116/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1116<F: Float>(t34205: F, t34207: F, t34209: F, t34211: F, t34214: F, t34217: F, t34219: F, t34222: F, t34224: F, t34227: F, t34230: F, t34235: F, t34238: F, t34241: F, t34245: F, t34249: F, t34252: F, t34255: F, t34258: F, t34264: F, t34269: F, t34274: F) -> (F, F) {
    let t38039 = 0.8096354166666666667e-4 * t34205 + 0.40481770833333333336e-3 * t34207 + 0.6487109086417285278e-2 * t34209 + 0.49163213094075520838e-7 * t34211 + 0.43440462632258606772e-4 * t34214 - 0.44197102999375800016e-7 * t34217 - 0.31432979653156068972e-7 * t34219 - 0.19336232562226912507e-7 * t34222 - 0.27011279664738401692e-5 * t34224 + 0.1686740451388888889e-5 * t34227 - 0.14758978949652777779e-5 * t34230;
    let t38051 = 0.20596571349374880758e-5 * t34235 + 0.80043425406508130348e-8 * t34238 + 0.69504740211613770836e-3 * t34241 + 0.98326426188151041676e-8 * t34245 - 0.32775475396050347226e-8 * t34249 - 0.20596571349374880758e-4 * t34252 - 0.65550950792100694451e-8 * t34255 + 0.44197102999375800016e-7 * t34258 + 0.2651826179962548001e-6 * t34264 - 0.21914396903857167508e-6 * t34269 + 0.15716489826578034486e-7 * t34274;
    (t38039, t38051)
}
