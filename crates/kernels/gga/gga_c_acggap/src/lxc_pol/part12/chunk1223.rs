//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1223/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1223<F: Float>(t7990: F, t9436: F, t2176: F, t5379: F, t1614: F, t8111: F, t2131: F, t2132: F, t309: F, t9367: F, t33175: F, t7963: F, t9029: F) -> (F, F, F, F, F) {
    let t38015 = F::new(0.17347256376410398924e1) * t7990 * t9436;
    let t38018 = F::new(0.13170898365871023197e1) * t2176 * t5379;
    let t38019 = t8111 * t1614;
    let t38033 = F::new(0.17347256376410398924e1) * t2131 * t2132 * t9367 * t309;
    let t38036 = F::new(0.17347256376410398924e1) * t7963 * t33175 * t9029;
    (t38015, t38018, t38019, t38033, t38036)
}
