//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1315/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1315<F: Float>(t469: F, t6576: F, t11883: F, t1717: F, t105: F, t11708: F, t1215: F, t12278: F, t12281: F, t1264: F, t14674: F, t14678: F, t14680: F, t14683: F, t14688: F, t15201: F, t15204: F, t15206: F, t15208: F, t15210: F, t15214: F, t15218: F, t1674: F, t1679: F, t1713: F, t18858: F, t18861: F, t18866: F, t1914: F, t1938: F, t19543: F, t19565: F, t19595: F, t19631: F, t19662: F, t19696: F, t19914: F, t19915: F, t19916: F, t19917: F, t19918: F, t19970: F, t19972: F, t24499: F, t24540: F, t2831: F, t301: F, t3865: F, t3875: F, t446: F, t6425: F, t694: F, t811: F, t96: F) -> F {
    let t24551 = t6576 * t469;
    let t24555 = t1717 * t11883;
    let t24559 = t96 * t105 * (-F::new(0.65854491829355115987e0) * t3865 * t1938 + F::new(0.52683593463484092788e1) * t1215 * t6425 - t14683 + t24499 + F::new(0.13170898365871023197e1) * t14678 - F::new(0.65854491829355115987e0) * t14680 + t19595 - F::new(0.13170898365871023197e1) * t18861 + F::new(0.39512695097613069591e1) * t15208 - F::new(0.52683593463484092788e1) * t15210 + F::new(0.52683593463484092788e1) * t15201 - F::new(0.52683593463484092788e1) * t15214 - F::new(0.39512695097613069592e1) * t14674 + t19565 - F::new(0.52683593463484092788e1) * t18858 - t12281 - F::new(0.26341796731742046394e1) * t15218 + t19631 - F::new(0.79025390195226139182e1) * t12278 + t19543 + F::new(0.65854491829355115987e0) * t14688 + t19662 + t19696 + t24540 + F::new(0.13170898365871023197e1) * t18866 - F::new(0.13170898365871023197e1) * t15204 + F::new(0.52683593463484092788e1) * t15206 - F::new(0.39512695097613069591e1) * t446 * t3875 * t1914 * t1264) * t469 + t19914 - t19915 + t19916 + t19917 + t11708 - t19918 + F::new(6.0) * t1674 * t2831 * t1713 + F::new(6.0) * t694 * t24551 * t301 - F::new(6.0) * t1679 * t24555 * t811 + t19970 + t19972;
    t24559
}
