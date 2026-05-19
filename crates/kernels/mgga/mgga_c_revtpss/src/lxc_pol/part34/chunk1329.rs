//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1329/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1329<F: Float>(t108395: F, t108435: F, t108438: F, t108440: F, t108455: F, t108464: F, t108474: F, t1904: F, t27837: F, t30089: F, t94865: F, t94867: F, t98084: F, t98099: F, t98101: F, t98104: F, t98312: F) -> F {
    let t114718 = -F::cast_from(0.77108554593144223218e-1_f64) * t108435 - F::cast_from(0.86736281882051994623e-1_f64) * t108438 + F::cast_from(0.15421710918628844643e0_f64) * t108440 - t94865 + F::cast_from(0.58544643236296698113e-1_f64) * t108455 - t94867 - F::cast_from(0.68549505033305214441e-2_f64) * t98084 - F::cast_from(0.43368140941025997312e-1_f64) * t108464 + F::cast_from(0.13010442282307799193e1_f64) * t27837 * t30089 - F::cast_from(0.72280234901709995519e-3_f64) * t98099 - F::cast_from(0.19756347548806534796e1_f64) * t108395 * t1904 - F::cast_from(0.51405703062096148812e-1_f64) * t98101 + F::cast_from(0.15421710918628844643e0_f64) * t108474 - F::cast_from(0.28912093960683998208e-1_f64) * t98104 + F::cast_from(0.68549505033305214441e-2_f64) * t98312;
    t114718
}
