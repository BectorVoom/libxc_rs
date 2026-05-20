//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2228/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2228<F: Float>(t100403: F, t1089: F, t16183: F, t16295: F, t16314: F, t1983: F, t1984: F, t1985: F, t25461: F, t25634: F, t25651: F, t25687: F, t25692: F, t25699: F, t27415: F, t27423: F, t27579: F, t27580: F, t27616: F, t27621: F, t27634: F, t3325: F, t3326: F, t359: F, t4742: F, t4941: F, t5016: F, t7135: F, t7144: F, t7145: F, t7151: F, t7160: F, t7167: F, t7168: F, t7821: F, t7829: F, t93436: F, t93498: F, t94068: F, t999: F, t99953: F, t99969: F, t99970: F) -> F {
    let t100425 = F::cast_from(0.13170898365871023197e1_f64) * t25692 * t4941 - F::cast_from(0.26341796731742046394e1_f64) * t99953 * t16314 - F::cast_from(0.17347256376410398924e1_f64) * t7144 * t7145 * t7135 * t4742 - F::cast_from(0.17347256376410398924e1_f64) * t7151 * t7160 * t7821 * t3325 - F::cast_from(0.17347256376410398924e1_f64) * t27415 * t27423 - F::cast_from(0.65854491829355115987e0_f64) * t27616 * t3326 - F::cast_from(0.10408353825846239354e2_f64) * t99969 * t1985 * t99970 * t999 - F::cast_from(0.52041769129231196772e1_f64) * t25699 * t7145 * t27579 * t999 - F::cast_from(0.4336814094102599731e0_f64) * t1983 * t1984 * t359 * t100403 - F::cast_from(0.13170898365871023197e1_f64) * t25634 * t5016 - F::cast_from(0.4336814094102599731e0_f64) * t27621 * t25687 + F::cast_from(0.13170898365871023197e1_f64) * t25651 * t16295 + F::cast_from(0.17347256376410398924e1_f64) * t25461 * t27580 + F::cast_from(0.34694512752820797848e1_f64) * t93436 * t27634 * t93498 + F::cast_from(0.17347256376410398924e1_f64) * t94068 * t7829 - F::cast_from(0.4336814094102599731e0_f64) * t7167 * t7168 * t16183 * t1089;
    t100425
}
