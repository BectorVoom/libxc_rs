//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1097/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1097<F: Float>(t11512: F, t14541: F, t1459: F, t1649: F, t3635: F, t8419: F, t35668: F, t35670: F, t35672: F, t35674: F, t35676: F, t35680: F, t35685: F, t35689: F, t35694: F, t35697: F, t35700: F, t35702: F) -> (F,) {
    let t35706 = t14541 * t1459 * t11512 * t1649;
    let t35708 = t8419 * t3635;
    let t35710 = 0.12147342662753799615e-3 * t35668 + 0.86898242813537603824e-4 * t35670 - 0.59742541934307102628e-4 * t35672 - 0.1545050757224698596e-4 * t35674 + 0.84356546269123608434e-6 * t35676 + 0.24603992661827719126e-6 * t35680 - 0.16146370184324440677e-6 * t35685 + 0.5431140175846100239e-5 * t35689 + 0.515016919074899532e-6 * t35694 - 0.86898242813537603824e-5 * t35697 - 0.86898242813537603824e-5 * t35700 + 0.7210236867048593448e-4 * t35702 + 0.57231255132198210494e-7 * t35706 - 0.52638484871933131664e-3 * t35708;
    (t35710,)
}
