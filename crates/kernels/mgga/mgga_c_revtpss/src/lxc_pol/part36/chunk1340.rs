//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1340/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1340<F: Float>(t108395: F, t108435: F, t108438: F, t108440: F, t108455: F, t108464: F, t108474: F, t1904: F, t27837: F, t30089: F, t94865: F, t94867: F, t98084: F, t98099: F, t98101: F, t98104: F, t98312: F) -> F {
    let t114718 = -F::new(0.77108554593144223218e-1) * t108435 - F::new(0.86736281882051994623e-1) * t108438 + F::new(0.15421710918628844643e0) * t108440 - t94865 + F::new(0.58544643236296698113e-1) * t108455 - t94867 - F::new(0.68549505033305214441e-2) * t98084 - F::new(0.43368140941025997312e-1) * t108464 + F::new(0.13010442282307799193e1) * t27837 * t30089 - F::new(0.72280234901709995519e-3) * t98099 - F::new(0.19756347548806534796e1) * t108395 * t1904 - F::new(0.51405703062096148812e-1) * t98101 + F::new(0.15421710918628844643e0) * t108474 - F::new(0.28912093960683998208e-1) * t98104 + F::new(0.68549505033305214441e-2) * t98312;
    t114718
}
