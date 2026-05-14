//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1352/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1352<F: Float>(t10310: F, t10316: F, t10323: F, t10352: F, t10353: F, t10356: F, t1250: F, t19245: F, t23498: F, t2363: F, t2443: F, t26927: F, t26936: F, t26940: F, t26948: F, t3260: F, t3266: F, t6455: F, t6514: F, t6523: F, t7832: F, t8445: F, t8508: F, t8512: F, t8520: F, t8543: F, t8549: F, t946: F) -> (F,) {
    let t26960 = 0.13170898365871023197e1 * t26927 * t946 + 0.65854491829355115987e0 * t10352 * t7832 * t8445 - 0.79025390195226139182e1 * t6523 * t10323 * t8520 + 0.52683593463484092788e1 * t2363 * t26936 * t3260 + 0.13170898365871023197e1 * t6455 * t26940 * t10353 + 0.65854491829355115987e0 * t10356 * t2443 + 0.52683593463484092788e1 * t8512 * t10316 + 0.65854491829355115987e0 * t26948 * t8543 + 0.39512695097613069591e1 * t10310 * t19245 + 0.79025390195226139182e1 * t6514 * t10323 * t8508 + 0.26341796731742046394e1 * t8549 * t3266 + 0.13170898365871023197e1 * t23498 * t1250;
    (t26960,)
}
