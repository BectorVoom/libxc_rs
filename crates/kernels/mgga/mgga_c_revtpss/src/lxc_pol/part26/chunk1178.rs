//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1178/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1178<F: Float>(t93377: F, t95726: F, t10495: F, t10977: F, t1956: F, t1957: F, t2061: F, t233: F, t25383: F, t26489: F, t7070: F, t7071: F, t7403: F, t95624: F, t95629: F, t95632: F, t95635: F, t95645: F, t95647: F, t95649: F, t95651: F, t95715: F, t95720: F, t95722: F) -> F {
    let t95727 = t93377 * t95726;
    let t95729 = F::new(0.15421710918628844643e0) * t95624 + F::new(0.39512695097613069591e1) * t7403 * t10495 - F::new(0.10281140612419229762e0) * t95629 + t95632 - F::new(0.16463622957338778996e-1) * t95635 - F::new(0.78062653693846795158e1) * t25383 * t26489 + F::new(0.8673628188205199462e0) * t7070 * t7071 * t2061 * t10977 + F::new(0.21684070470512998656e-1) * t95645 - F::new(0.38554277296572111609e-1) * t95647 + F::new(0.77108554593144223218e-1) * t95649 - F::new(0.29272321618148349057e-1) * t95651 - F::new(0.4336814094102599731e0) * t1956 * t1957 * t233 * t95715 - F::new(0.43368140941025997312e-1) * t95720 + F::new(0.57824187921367996415e-1) * t95722 - F::new(0.10281140612419229763e-1) * t95727;
    t95729
}
