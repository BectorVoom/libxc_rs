//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 960/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk960<F: Float>(t10201: F, t10205: F, t10208: F, t10213: F, t10217: F, t10220: F, t10223: F, t10227: F, t10232: F, t10234: F, t10238: F, t10247: F, t10250: F, t10253: F, t10258: F, t10262: F, t10267: F, t10271: F, t10274: F, t10276: F, t10278: F, t10281: F) -> (F, F) {
    let t11097 = -F::new(0.1487444284829289667e-3) * t10201 + F::new(0.23485962392041415794e-4) * t10205 + F::new(0.11742981196020707897e-4) * t10208 + F::new(0.11273261948179879581e-2) * t10213 - F::new(0.66812865812879419652e-4) * t10217 - F::new(0.11742981196020707897e-4) * t10220 - F::new(0.685007236434541294e-5) * t10223 - F::new(0.685007236434541294e-5) * t10227 - F::new(0.40598095546020480691e-6) * t10232 - F::new(0.23485962392041415794e-4) * t10234 - F::new(0.11742981196020707897e-4) * t10238;
    let t11111 = F::new(0.13919347044349879094e-6) * t10247 - F::new(0.41758041133049637282e-5) * t10250 - F::new(0.23485962392041415794e-5) * t10253 - F::new(0.14984533297005590806e-5) * t10258 - F::new(0.24748599044854085031e-6) * t10262 + F::new(0.41758041133049637282e-5) * t10267 - F::new(0.23485962392041415794e-4) * t10271 - F::new(0.66812865812879419652e-4) * t10274 + F::new(0.1487444284829289667e-3) * t10276 + F::new(0.46808827823026988424e-4) * t10278 - F::new(0.23485962392041415794e-5) * t10281;
    (t11097, t11111)
}
