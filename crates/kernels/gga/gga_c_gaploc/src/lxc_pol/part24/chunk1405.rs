//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1405/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1405<F: Float>(t34826: F, t10421: F, t20887: F, t10424: F, t30733: F, t10569: F, t10570: F, t10573: F, t10591: F, t1580: F, t1628: F, t3358: F, t34790: F, t34794: F, t34797: F, t34801: F, t34817: F, t34821: F, t34823: F, t4585: F, t541: F, t557: F, t597: F) -> F {
    let t34827 = F::new(0.89376224879626066674e-1) * t34826;
    let t34828 = t10421 * t20887;
    let t34829 = F::new(0.14896037479937677779e-1) * t34828;
    let t34830 = t10424 * t30733;
    let t34831 = F::new(0.59584149919750711116e-1) * t34830;
    let t34832 = t34790 - t34794 - t34797 - t34801 + F::new(0.46011511144704899612e1) * t1580 * t10570 + F::new(0.47667319935800568892e0) * t10591 * t541 + F::new(0.79445533226334281487e-1) * t557 * t4585 * t3358 + F::new(0.61348681526273199482e1) * t1580 * t10573 + F::new(0.61348681526273199482e1) * t597 * t1628 * t10569 + t34817 - t34821 + t34823 - t34827 - t34829 - t34831;
    t34832
}
